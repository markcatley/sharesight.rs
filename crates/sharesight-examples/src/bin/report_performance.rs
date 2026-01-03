use std::num::NonZeroU32;

use chrono::{prelude::*, Duration};
use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{
    GroupsList, GroupsListGroupsSuccess, GroupsListSuccess, PerformanceShow,
    PerformanceShowParameters, PerformanceShowSuccess,
};

/// Generate a 'performance' report using the sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The names of the portfolios to report the performance for.
    #[clap(num_args = 1..)]
    portfolio_names: Vec<String>,
    /// Lookback periods in years
    #[clap(short, default_value = "5", num_args = 1..)]
    look_back_periods_in_years: Vec<NonZeroU32>,
    /// Display performance for each group in the grouping
    #[clap(short)]
    group: Option<String>,
    /// The access token to use the api.
    /// JSON file including api host, client_id and client_secret.
    #[clap(short)]
    client_credentials_file: std::path::PathBuf,
    /// The access token to use the api.
    #[clap(short)]
    user_credentials_file: std::path::PathBuf,
    // Inception date to use, defaults to the earliest portfolio inception date.
    #[clap(long)]
    inception_date: Option<NaiveDate>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    let args = Args::parse();
    let client = Client::new(args.user_credentials_file, args.client_credentials_file).await?;
    let portfolio_names = args.portfolio_names;
    let group_name = args.group;
    let look_back_periods_in_years = args
        .look_back_periods_in_years
        .iter()
        .map(|l| l.get() as i32)
        .collect::<Vec<_>>();

    let portfolio_index = client.build_portfolio_index().await?;
    let portfolios = portfolio_names
        .iter()
        .map(|portfolio_name| {
            portfolio_index.find(portfolio_name).unwrap_or_else(|| {
                portfolio_index.log_error_for(portfolio_name);
                std::process::exit(0);
            })
        })
        .collect::<Vec<_>>();

    let groups = client.execute::<GroupsList, GroupsListSuccess>(&()).await?;
    let group = group_name.map(|group_name| {
        groups
            .find_group(&group_name)
            .unwrap_or_else(|| std::process::exit(0))
    });
    let custom_group_id = group.and_then(|g| g.id.id());
    let grouping = if custom_group_id.is_some() {
        Some("custom_group".to_string())
    } else {
        group.and_then(|g| g.id.name())
    };

    let today = Utc::now().date_naive();
    let inception_on = args.inception_date.unwrap_or_else(|| {
        portfolios
            .iter()
            .map(|portfolio| portfolio.inception_date)
            .min()
            .unwrap()
    });
    let mut end_of_current_period = today.start_of_next_quarter() - Duration::days(1);
    let mut start_of_current_periods = look_back_periods_in_years
        .iter()
        .copied()
        .map(|look_back_period_in_years| {
            (
                look_back_period_in_years,
                end_of_current_period
                    .start_of_next_quarter()
                    .add_years(-look_back_period_in_years),
            )
        })
        .collect::<Vec<_>>();

    let grouping_titles = {
        let mut titles = Vec::new();

        for portfolio in grouping.iter().flat_map(|_| portfolios.iter()).copied() {
            let performance_parameters = PerformanceShowParameters {
                start_date: None,
                end_date: None,
                portfolio_id: portfolio.id,
                consolidated: portfolio.consolidated,
                include_sales: Some(true),
                report_combined: None,
                labels: None,
                grouping: grouping.clone(),
                custom_group_id,
                include_limited: None,
            };
            let PerformanceShowSuccess {
                report: performance_report,
                ..
            } = client
                .execute::<PerformanceShow, _>(&performance_parameters)
                .await?;

            titles.extend(
                performance_report
                    .sub_totals
                    .into_iter()
                    .map(|s| s.group_name)
                    .filter(|t| !titles.contains(t))
                    .collect::<Vec<_>>(),
            )
        }

        titles
    };

    print!("End Date,Total");
    for _ in 0..(start_of_current_periods.len() * portfolios.len() - 1) {
        print!(",");
    }

    for title in &grouping_titles {
        print!(",{}", title);
        for _ in 0..(start_of_current_periods.len() * portfolios.len() - 1) {
            print!(",");
        }
    }
    println!();

    for _ in 0..(grouping_titles.len() + 1) {
        for (period, _) in start_of_current_periods.iter() {
            print!(",{} year", period);
            for _ in portfolios.iter().skip(1) {
                print!(",");
            }
        }
    }
    println!();

    for _ in 0..((grouping_titles.len() + 1) * start_of_current_periods.len()) {
        for portfolio in &portfolios {
            print!(",{}", portfolio.name);
        }
    }
    println!();

    while start_of_current_periods
        .iter()
        .map(|(_, d)| d)
        .copied()
        .max()
        .unwrap()
        >= inception_on
    {
        let mut reports = Vec::new();

        for (_, start_of_current_period) in start_of_current_periods.iter().copied() {
            for portfolio in &portfolios {
                if start_of_current_period >= inception_on {
                    let performance_parameters = PerformanceShowParameters {
                        start_date: Some(start_of_current_period),
                        end_date: Some(end_of_current_period),
                        portfolio_id: portfolio.id,
                        consolidated: portfolio.consolidated,
                        include_sales: Some(true),
                        grouping: grouping.clone(),
                        custom_group_id,
                        include_limited: None,
                        report_combined: None,
                        labels: None,
                    };
                    let PerformanceShowSuccess { report, .. } = client
                        .execute::<PerformanceShow, _>(&performance_parameters)
                        .await?;

                    reports.push(Some(report));
                } else {
                    reports.push(None);
                }
            }
        }

        print!("{}", end_of_current_period);

        for report in &reports {
            if let Some(report) = report {
                print!(",{}%", report.total_gain_percent);
            } else {
                print!(",");
            }
        }

        for title in &grouping_titles {
            for performance_report in &reports {
                let sub_total = performance_report
                    .iter()
                    .flat_map(|r| r.sub_totals.iter())
                    .find(|sub_total| &sub_total.group_name == title);
                if let Some(sub_total) = sub_total {
                    print!(",{}%", sub_total.total_gain_percent);
                } else {
                    print!(",");
                }
            }
        }

        println!();

        for (_, start_of_current_period) in start_of_current_periods.iter_mut() {
            *start_of_current_period = start_of_current_period.start_of_last_quarter();
        }
        end_of_current_period = end_of_current_period.end_of_last_quarter();
    }

    Ok(())
}

trait DateExt {
    fn start_of_quarter(&self) -> Self;
    fn start_of_next_quarter(&self) -> Self;
    fn start_of_last_quarter(&self) -> Self;
    fn end_of_last_quarter(&self) -> Self;
    fn add_years(&self, years: i32) -> Self;
}

impl DateExt for NaiveDate {
    fn start_of_quarter(&self) -> Self {
        NaiveDate::from_ymd_opt(self.year(), self.month() - ((self.month() - 1) % 3), 1).unwrap()
    }

    fn start_of_next_quarter(&self) -> Self {
        let start_of_quarter = self.start_of_quarter();
        let (year, month, day) = (
            start_of_quarter.year(),
            start_of_quarter.month() + 3,
            start_of_quarter.day(),
        );
        let (year, month) = if month > 12 {
            (year + 1, month - 12)
        } else {
            (year, month)
        };

        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    fn start_of_last_quarter(&self) -> Self {
        self.end_of_last_quarter().start_of_quarter()
    }

    fn end_of_last_quarter(&self) -> Self {
        self.start_of_quarter() - Duration::days(1)
    }

    fn add_years(&self, years: i32) -> Self {
        let (year, month, day) = (self.year() + years, self.month(), self.day());

        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }
}

trait GroupsListSuccessExt {
    fn find_group<'a>(&'a self, group_name: &str) -> Option<&'a GroupsListGroupsSuccess>;
}

impl GroupsListSuccessExt for GroupsListSuccess {
    fn find_group<'a>(&'a self, group_name: &str) -> Option<&'a GroupsListGroupsSuccess> {
        let group = self.groups.iter().find(|p| p.name == group_name);

        if let Some(group) = group {
            Some(group)
        } else {
            eprint!("Unknown group: {}, ", group_name);

            let mut names = self.groups.iter().map(|p| p.name.as_str());

            match (names.next(), names.next_back()) {
                (Some(name_start), Some(name_end)) => {
                    eprint!("the group are: {}", name_start);
                    for name in names {
                        eprint!(", {}", name);
                    }
                    eprintln!(" or {}", name_end);
                }
                (Some(name), None) => {
                    eprintln!("the only portfolio is: {}", name);
                }
                (None, None) => {
                    eprintln!("there are no portfolios");
                }
                _ => unreachable!(),
            }

            None
        }
    }
}
