pub fn display(jobs: Option<&str>) -> Option<String> {
    jobs.map(|jobs| jobs.to_owned())
}
