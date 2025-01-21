use court_api::constitutional_court::{
    BenchType, ConstitutionCaseType as CaseType, ConstitutionalCaseSearchParameter as SearchParam,
};

#[tokio::main]
async fn main() {
    let search = SearchParam::default()
        .set_bench_type(vec![BenchType::EnBancBench])
        .set_limit(50)
        .search()
        .await
        .unwrap();

    println!("{search:#?}");
}
