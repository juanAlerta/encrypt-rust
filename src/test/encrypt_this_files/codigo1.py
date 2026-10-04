# hash_dir.py -----------------------------------------------------------
# -----------------------------------------------------------------------
# This script prints the hashes of the files in the current directory
# and execute a request to VirusTotal to 
# obtain a diagnosis of the suspicious file
# 
# author: github.com/juanAlerta
# -----------------------------------------------------------------------
#  TO-DO ----------------------------------------------------------------
# - Sacar API KEY
# - Meter colorines ✅
# - Cambiar a SHA256 ✅
# -----------------------------------------------------------------------

import hashlib
import os
import requests
import sys

RED = "\033[91m"
YELLOW = "\033[93m"
GREEN = "\033[92m"
RESET = "\033[0m"
    
keywords = {
    "unknown file": YELLOW,
    "malicious": RED,
    "harmless": GREEN,
}

api_key = "key" # https://www.virustotal.com/gui/user/USER_NAME/apikey

if len(sys.argv) > 1:
    current_dir = sys.argv[1]
else:
    current_dir = os.getcwd()

def value_from_key(dicc, value):
    for key, actual_value in dicc.items():
        if actual_value == value:
            return key
    return None

def calculate_file_hashes():
    hashes = {}
    
    for file in os.listdir(current_dir):
        if os.path.isfile(file):
            with open(file, 'rb') as f:
                sha256 = hashlib.sha256()
                sha256.update(f.read())
                hashes[f.name] = sha256.hexdigest()
    
    return hashes

def vt_diagnosis(hashes):
    headers = {
        "accept": "application/json",
        "x-apikey": api_key
    }
    
    for h in hashes.values():
        url = f"https://www.virustotal.com/api/v3/files/{h}/votes"
        response = requests.get(url, headers=headers) # verify=False    
        
        key = value_from_key(hashes, h)
        
        if response.status_code == 200:
            parser(response.text, key, h)
        else:
            print(f"{h}  {key} is a {YELLOW}unknown file 😶‍🌫️{RESET}")

def parser(response, key, value):
    if "malicious" in response:
        print(f"{value}  {key} is {RED}malicious 😈{RESET}")
    else:
        print(f"{value}  {key} is {GREEN}harmless 😇{RESET}")

file_hashes = calculate_file_hashes()
vt_diagnosis(file_hashes)