#!/usr/bin/ruby
require 'set'

networks = Hash.new
total = 0
f = File.open('access.log')

f.read.each_line do |line|
	ip = line.split(" - - ")[0]
	ip_parts = ip.split(".")
	net = ip_parts[0] + "." + ip_parts[1] + "." + ip_parts[2] + ".0"
	if networks.include?(net) == false then
		networks[net] = Set.new()
	end
	networks[net].add(ip)
	total += 1
end
print("Total: #{total}\n")
print("Unique: #{networks.values.map{ |x| x.count }.reduce(:+)}\n")
print("\n")

networks.each do |net, ips|
	print("#{net}:\n\t#{ips.to_a}\n")
end

f.close()
