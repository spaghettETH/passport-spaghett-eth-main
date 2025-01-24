STEPS_

- download the repo
- Follow READMEs instructions in src/ , app/styles/ , contract/scripts/
- navigate to contract/ folder and:
  - create .env file with: 

  API_URL=<your database API URL>
  SESSION=<your session token>

  - npm install
  - in checkpoints/ create a new <CSV_FILE_NAME>.csv file for a new event with batch of talk check-ins (see example.csv)
  - navigate again to contract/ with terminal and run these 2 commands:

    - node scripts/api/create-qr-codes.js <CSV_FILE_NAME>
    - node scripts/api/store-checkpoints.js <CSV_FILE_NAME>
