import requests
import re
from urllib.parse import urlparse

# Globals

ROOT = "http://www.mosigra.ru"

MAX_PAGES = 10
PARSED_PAGES = 0

emails = set()
urls = set()

email_regex = "([\w]+[\w-]*(?:\.[\w-]+)*)@((?:[\w-]+\.)*\w[\w-]{0,66})\.([a-z]{2,6}(?:\.[a-z]{2})?)"
url_regex = r'href=[\'"]?([^\'" >]+)'

def domain(hostname):
	tmp = hostname.split(".")
	if len(tmp) < 2:
		return None
	return tmp[-2] + "." + tmp[-1]

def match_to_email(x):
	return (x[0] + "@" + x[1] + "." + x[2]).lower()

def match_to_url(x, parent):
	result = "".join(x).lower()
	# Filter file URLs
	if not (result.endswith("/") or result.endswith(".html") or result.endswith(".htm")):
		return None
	# Expand relative URLs
	if urlparse(result).scheme == '':
		if result.startswith("/"):
			result = ROOT + result
		elif result.startswith("./"):
			result = parent + result
	# Filter external URLs
	if not result.startswith(ROOT):
		return None
	return result

def lookup_url(url):
	print ("\nPARSING: ", url)
	r = requests.get(url)
	if not r.status_code == 200:
		print("Error:", r.status_code)
		return
	# Fetch emails
	emails.update(map(match_to_email, re.findall(email_regex, r.text)))
	print("\n", sorted(emails))
	# Fetch all hrefs
	raw_links = [match_to_url(x, url) for x in re.findall(url_regex, r.text)]
	linked_pages = set(filter(None, raw_links))
	# Find new urls
	new_pages = linked_pages.difference(urls)
	urls.update(new_pages)
	# Traverse them (if needed)
	global PARSED_PAGES
	PARSED_PAGES += 1
	if PARSED_PAGES >= MAX_PAGES:
		raise Exception("Done")
	for page in new_pages:
		if PARSED_PAGES < MAX_PAGES:
			lookup_url(page)
	
# Start
try:
	lookup_url(ROOT)
except:
	pass
finally:
	print("\n\n\nRESULTS:")
	for e in emails:
		print("\t", e)
