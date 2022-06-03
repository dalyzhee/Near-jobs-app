use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
// use near_sdk::serde::Serialize;
use near_sdk::env;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Job {
    title: String,
    description: String,
    location: String,
    date: String,
    email: String,
    company: String
}
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct JobsMarket {
  jobs: Vec<Job>,
}


#[near_bindgen]
impl JobsMarket {
    // ADD CONTRACT METHODS HERE
    #[init]
    pub fn new_job() -> Self{
        let jobs: Vec<Job> = Vec::new();

        JobsMarket{
            jobs
        }
    }

    pub fn job_count(&mut self) -> usize{
        self.jobs.len()
    }

    pub fn add_job(&mut self, title: String,
        description: String,
        location: String,
        date: String,
        email: String,
        company: String){
            let job1 = Job{
                title: title.to_string(),
                description: description.to_string(),
                location: location.to_string(),
                date: date.to_string(),
                email: email.to_string(),
                company: company.to_string()
            };
            self.jobs.push(job1);
            env::log_str("Job added");
        }

    pub fn show_job(&mut self) -> &Vec<Job>{
        &self.jobs
    }
    pub fn delete_job(&mut self){
        self.jobs.pop();
        env::log_str("Job deleted");
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(account: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(account);
        return builder;
    }

    // TESTS HERE
    #[test]
    fn job_existence(){
        let user = AccountId::new_unchecked("dalyzhee.testnat".to_string());
        let _context = get_context(user.clone());
        let mut job = JobsMarket::new_job();
        job.add_job("software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string());
        let counting = job.job_count();
        assert_eq!(counting, 1);
    }
    #[test]
    fn add_job(){
        let user = AccountId::new_unchecked("dalyzhee.testnat".to_string());
        let _context = get_context(user.clone());

        let mut job = JobsMarket::new_job();
        
        job.add_job("software engineer".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string());
        job.add_job("web developer".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string());
        job.add_job("designer".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string());
        let counts = job.job_count();
        assert_eq!(counts, 3);
    }

    #[test]
    fn get_job(){
        let user = AccountId::new_unchecked("dalyzhee.testnat".to_string());
        let _context = get_context(user.clone());
        let mut job = JobsMarket::new_job();
        job.add_job("software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string());
        let counts = job.show_job();
        assert_eq!(counts.len(), 1);
    }

    #[test]
    fn delete_job(){
        let user = AccountId::new_unchecked("dalyzhee.testnat".to_string());
        let _context = get_context(user.clone());
        let mut job = JobsMarket::new_job();
        job.add_job("software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string());
        job.delete_job();
        let counts = job.show_job();
        assert_eq!(counts.len(), 0);
    }
}
