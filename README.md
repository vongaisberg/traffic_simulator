# Traffic Simulator
Simulate realistic network traffic for testing purposes.
Can handle about 3Gbit/s per core.

## Usage
> `traffic_simulator 1000 127.0.0.1:60000`
> Send 1Gbit/s (on average) of realistic traffic to localhost

## Literature
[Varet, Larrieu (2014)](https://enac.hal.science/hal-00973913/document) describes how network traffic looks like overlapping packet trains which have are spaced according to a Weibull (heavy-tail) distribution.

![image](https://github.com/user-attachments/assets/ea468e58-409d-4514-8843-39583f57da3b)

<sup>  Varet, Larrieu (2014)</sup>



[John, Tafvelin (2007)](http://conferences.sigcomm.org/imc/2007/papers/imc91.pdf) found that network packet sizes are bimodally distributed around 1500 bytes (Ethernet MTU) and 150-200 bytes (HTTP Requests and TCP ACKs).

![image](https://github.com/user-attachments/assets/fb49d21b-620f-45a1-b106-6e900681d82a)

<sup>  John, Tafvelin (2007)</sup>



