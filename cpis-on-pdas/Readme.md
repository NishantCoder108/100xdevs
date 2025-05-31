# CPI on PDA

CPI (Cross Program Invocation), one program call to other program internally thats method called cpi.


### What i Learned
- PDA is deterministically derivable ,means from same seed and program id ,it will generate same program derived address. 
- **find_program_address** function that will derive `pda` and `bump`
- For creating account for specific PDA , so we will create instruction using `create_account` and here we will pass `owner` that is `program_id` . may be of your programs' id or different one's program_id , it will create account on that program id.
- After instruction ,we will `invoke_signed` function , where we will pass `seed` 
- After successfully , i have created account on specific program.
