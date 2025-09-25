use std::num::NonZeroU32;

use chrono::{prelude::*, Duration};
use clap::Parser;
use sharesight_examples::init_logger;
use sharesight_reqwest::Client;
use sharesight_types::{
    PerformanceShow, PerformanceShowParameters, PerformanceShowSuccess, DEFAULT_API_HOST,
};

/// Generate a 'performance' report using the sharesight API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The host to use to access the API.
    #[clap(long, default_value = DEFAULT_API_HOST)]
    api_host: String,
    /// The name of the portfolio to report the performance for.
    portfolio_name: String,
    /// Lookback period in years
    #[clap(short, default_value = "5")]
    look_back_period_in_years: NonZeroU32,
    /// The access token to use the api.
    access_token: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    let args = Args::parse();
    let client = Client::new_with_token_and_host(args.access_token, args.api_host);
    let portfolio_name = args.portfolio_name;
    let look_back_period_in_years = args.look_back_period_in_years.get() as i32;

    let portfolios = client.build_portfolio_index().await?;

    let portfolio = portfolios.find(&portfolio_name).unwrap_or_else(|| {
        portfolios.log_error_for(&portfolio_name);
        std::process::exit(0);
    });

    let today = Utc::now().date_naive();
    let inception_on = portfolio.inception_date;
    let mut end_of_current_period = today.start_of_next_quarter() - Duration::days(1);
    let mut start_of_current_period = end_of_current_period
        .start_of_next_quarter()
        .add_years(-look_back_period_in_years);

    println!("Start Date,End Date,Gain/Loss,Gain/Loss (%)");

    while start_of_current_period >= inception_on {
        let performance_parameters = PerformanceShowParameters {
            start_date: Some(start_of_current_period),
            end_date: Some(end_of_current_period),
            portfolio_id: portfolio.id,
            consolidated: portfolio.consolidated,
            include_sales: Some(true),
            grouping: None,
            custom_group_id: None,
            include_limited: None,
            report_combined: None,
            labels: None,
        };
        let PerformanceShowSuccess {
            report: performance_report,
            ..
        } = client
            .execute::<PerformanceShow, _>(&performance_parameters)
            .await?;

        println!(
            "{},{},{},{}",
            performance_report.start_date,
            performance_report.end_date,
            performance_report.total_gain,
            performance_report.total_gain_percent
        );

        start_of_current_period = start_of_current_period.start_of_last_quarter();
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
