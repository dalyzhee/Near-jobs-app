
##  getting the number of jobs in the vector
near call leviso.testnet job_count --accountId youraccount.testnet

##  adding job to the vector 
near call leviso.testnet add_job '{"title": "some todo value"}' --accountId youraccount.testnet

##  displaying the jobs available 
near call leviso.testnet show_job --accountId youraccount.testnet

##  deleting jobs from the vector
near call leviso.testnet delete_job --accountId youraccount.testnet