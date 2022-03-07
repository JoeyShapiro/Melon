from scapy.all import *

'''
    take in a pcap file and return a list of packets (in bytes)
'''
def pcap2list(file):
    pcap = PcapReader(file).read_all() # get the file and read all the packets

    packets = []

    for pkt in pcap: # for each packet in pcap file
        packets.append(pkt) # add to packets list
    
    return packets # return bytes list of packets

####### EXAMPLE ########
ps = pcap2list('http.cap')