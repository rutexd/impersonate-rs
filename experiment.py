from curl_cffi import requests
import time

url = "https://www.idealista.com/inmueble/108979804/"

# List of browsers supported by curl_cffi 0.7+ (based on the file we read earlier)
# Some might not be valid in the specific installed version, we'll wrap in try/except
browsers = [
    # Chrome
    "chrome99", "chrome100", "chrome101", "chrome104", "chrome107", 
    "chrome110", "chrome116", "chrome119", "chrome120", "chrome123", "chrome124",
    # Android
    "chrome99_android",
    # Edge
    "edge99", "edge101",
    # Safari
    "safari15_3", "safari15_5", "safari17_0", "safari17_2_ios",
    # Firefox is rare in older versions but let's try generic fallback if mapped
]

print(f"Testing target: {url}")
print(f"Total profiles to test: {len(browsers)}")
print("-" * 60)
print(f"{'PROFILE':<20} | {'STATUS':<6} | {'RESULT'}")
print("-" * 60)

for browser in browsers:
    try:
        # Add a small delay to avoid rate limiting the scanner itself
        time.sleep(1)
        
        response = requests.get(
            url, 
            impersonate=browser,
            timeout=10
        )
        
        status = response.status_code
        
        # Check if we got something other than the challenge
        # The challenge usually has <p id="cmsg">Please enable JS...</p>
        is_challenge = "Please enable JS" in response.text or "idealista.com" in response.text
        
        result_msg = "CHALLENGE" if is_challenge else "POSSIBLE BYPASS"
        if status == 200 and not is_challenge:
            result_msg = "!!! SUCCESS !!!"
            
        print(f"{browser:<20} | {status:<6} | {result_msg}")
        
    except Exception as e:
        print(f"{browser:<20} | ERROR  | {str(e)[:40]}...")

print("-" * 60)
