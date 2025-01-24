# PASSPORT  

**PASSPORT** is a Proof of Attendance protocol developed by **Yomi.Digital** and **Spaghett-eth.com**.  
It empowers conference organizers to create check-ins for talks, workshops, or sessions in their schedule.  

For each check-in, QR codes are generated. These QR codes enable attendees to mint a **Passport SBT** (Soulbound Token) and update the SBT metadata with the corresponding check-in information.  

---

## How to Use  

### Steps  

1. **Clone the Repository**  
   Download the repository to your local machine.  

2. **Follow Setup Instructions**  
   Refer to the README files in the following folders for detailed setup instructions:  
   - `src/`  
   - `app/styles/`  
   - `contract/scripts/`  

3. **Set Up the Contract Folder**  
   Navigate to the `contract/` folder and:  
   
   - Create a `.env` file with the following variables:  
     ```plaintext  
     API_URL=<your database API URL>  
     SESSION=<your session token>  
     ```  
   
   - Run the following command to install dependencies:  
     ```bash  
     npm install  
     ```  

   - In the `checkpoints/` folder, create a new CSV file (`<CSV_FILE_NAME>.csv`) to define the event’s check-ins. Use the provided `example.csv` as a reference for formatting.  

4. **Generate and Store QR Codes**  
   Use the terminal to navigate back to the `contract/` folder and execute the following commands:  

   - To generate QR codes for the check-ins:  
     ```bash  
     node scripts/api/create-qr-codes.js <CSV_FILE_NAME>  
     ```  

   - To store the generated checkpoints:  
     ```bash  
     node scripts/api/store-checkpoints.js <CSV_FILE_NAME>  
     ```  
