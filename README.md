# UDP-Integrity-Experiment
You always hear that UDP messages aren't reliable. They tell you to use UDP in situations where it's OK for packets to disappear randomly, or be corrupted, or be duplicated.

What I want to find out is, does that actually happen? In $CURRENT_YEAR?

Let's send a bunch of UDP datagrams and find out.


![image](https://user-images.githubusercontent.com/5377932/120906724-7ee3a280-c621-11eb-8759-bcc9a0b050c9.png)


## Results

| Experiment | Datagrams sent | Datagrams corrupted | Datagrams that didn't arrive |
| --- | --- | --- | --- | 
| LAN between two computers | 58,090,836 | 0 | 0 |
| Between South US and Canadian VPS | 111,256 | 0 | 0 |

## Usage
### Run server:
```./UdpSafety server {port}```

Example: ```./UdpSafety server 3555```

### Run client:
```./UdpSafety client {server ip}:{port}```

Example: ```./UdpSafety client 192.168.1.55:3555```
