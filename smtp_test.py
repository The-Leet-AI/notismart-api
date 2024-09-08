import smtplib
import os
from dotenv import load_dotenv
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText

# Load environment variables from the .env file
load_dotenv()

# SMTP configuration
SMTP_SERVER = os.getenv("SMTP_SERVER")
SMTP_PORT = int(os.getenv("SMTP_PORT"))
SMTP_USERNAME = os.getenv("SMTP_USERNAME")
SMTP_PASSWORD = os.getenv("SMTP_PASSWORD")

def send_test_email():
    # Create the email content
    msg = MIMEMultipart()
    msg['From'] = SMTP_USERNAME
    msg['To'] = SMTP_USERNAME  # Sending the email to yourself
    msg['Subject'] = "Test Email from Python"

    body = "This is a test email sent from Python script."
    msg.attach(MIMEText(body, 'plain'))

    try:
        # Set up the SMTP server connection
        server = smtplib.SMTP(SMTP_SERVER, SMTP_PORT)
        server.starttls()  # Start TLS encryption
        server.login(SMTP_USERNAME, SMTP_PASSWORD)
        
        # Send the email
        server.sendmail(SMTP_USERNAME, SMTP_USERNAME, msg.as_string())
        print("Email sent successfully")
    except Exception as e:
        print(f"Failed to send email: {e}")
    finally:
        server.quit()

# Run the function
send_test_email()
