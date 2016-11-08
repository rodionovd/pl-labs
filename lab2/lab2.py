"""
Необходимо написать скрипт, обрабатывающий лог файл Nginx и выводящий список IP адресов, с которых производились запросы. Адреса из общей подсети \24 необходимо группировать при выводе (например, 10.40.0.4 и 10.40.0.231 относятся к одной подсети).

Версия Python: 3.5
"""

import ipaddress

MASK = 24

f = open("access.log", "r")
total = 0
unique = 0
networks = dict()
ips = dict()
for line in f.readlines():
	# IP address string
	ip = line.split(" - - ")[0]
	# Containing network
	net = ipaddress.ip_network(ip + '/' + str(MASK), strict=False)
	# Remember this network and this particular IP address
	networks.setdefault(net, set())
	ips.setdefault(ip, 0)
	# Adjust counters
	unique += ip not in networks[net]
	total += 1
	networks[net].add(ip)
	ips[ip] += 1
	

print("Total: ", total)
print("Unique: ", unique)
print()

for k in networks:
	if len(networks[k]) == 1:
		ip = networks[k].pop()
		print(ip, " (" + str(ips[ip]) + ")")
	else:
		print(k)
		for ip in networks[k]:
			print("\t==> " + ip +  " (" + str(ips[ip]) + ")")