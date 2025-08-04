import os
import time
import datetime
import configparser
import garth
from tqdm import tqdm

# Get the directory of the current script
script_dir = os.path.dirname(os.path.abspath(__file__))

# Construct the full path to the config file
config_path = os.path.join(script_dir, 'config.ini')
garth_path = os.path.join(script_dir, '.garth')

# Read configuration
config = configparser.ConfigParser()
config.read(config_path)

# Garmin API URLs
garmin_connect_activity_search_url = "/activitylist-service/activities/search/activities"
garmin_connect_download_service_url = "/download-service/files"

# Read credentials and file folder from config
email = config['Garmin']['email']
password = config['Garmin']['password']
fit_file_folder = config['Garmin']['fit_file_folder']

# Garth authentication
try:
    garth.resume(garth_path)
    garth.client.username
except:
    garth.login(email, password)
    garth.save(garth_path)

# get start date
try:
    with open(f'{fit_file_folder}/last.txt', 'r') as file:
        line = file.read().strip()
        start_date = datetime.datetime.strptime(line, '%Y-%m-%d')
except FileNotFoundError:
    # If last.txt doesn't exist, start from a default date
    start_date = datetime.datetime.now() - datetime.timedelta(days=30)

# get end date (today)
end_date = datetime.date.today()

# Calculate the number of days between start and end dates
delta = end_date - start_date.date()
total_days = delta.days + 1

# Iterate through each day
for day in tqdm(range(total_days), desc="Downloading FIT files"):
    time.sleep(1)
    current_date = start_date + datetime.timedelta(days=day)
    date_str = current_date.strftime("%Y-%m-%d")

    params = {
        'startDate': date_str,
        "endDate": date_str
    }

    # get all activities for that day
    activities = garth.connectapi(garmin_connect_activity_search_url, params=params)
    
    for activity in activities:
        time.sleep(1)
        # get .fit file for each activity in one day
        activity_id = str(activity['activityId'])
        
        response = garth.client.get('connectapi', f'{garmin_connect_download_service_url}/activity/{activity_id}', api=True)
        filename = f'{fit_file_folder}/{activity_id}.zip'
        
        with open(filename, 'wb') as file:
            for chunk in response:
                file.write(chunk)

# update last download day to today in "last.txt"
current_date = datetime.date.today().strftime('%Y-%m-%d')
with open(f'{fit_file_folder}/last.txt', 'w') as file:
    file.write(current_date)
