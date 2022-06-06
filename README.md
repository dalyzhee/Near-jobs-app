# Near Jobs Site.

The smart contract code  is in src `lib.rs`.

This contract was inspired by the need for graduates or any person looking for ajob. This will act as a stop shop for everyone looking for a job.
It also help an organization to be able to diaplay their jobs.

This contract performs three major functions which include:
1. Adding jobs into vector.
2. Display the jobs.
3. Delete job once application date is over.

From this contract user will get the job title, decription, location, date, company and the email where he/she can send his/her cv or resume.

The contract contains tests.
The test are:
1. Test for existence of job in the vector.
2. Test for getting jobs for diplay.
3. Test for adding jobs to the vector.
4. Test for deleting a job from the vector. 


# Required Software

- Rust 1.58 + cargo
- Node.js
- NEAR CLI 3.1

## Quick near calls from the terminal

##  getting the number of jobs in the vector
near call leviso.testnet job_count --accountId youraccount.testnet

##  adding job to the vector 
near call leviso.testnet add_job '{"title": "software", "description": "software", "location": "nairobi", "date": "ten", "email": "dalmas", "company": "andela"}'  --accountId youraccount.testnet

##  displaying the jobs available 
near call leviso.testnet show_job --accountId youraccount.testnet

##  deleting jobs from the vector
near call leviso.testnet delete_job --accountId youraccount.testnet

# Authors
- Levis Odhiambo