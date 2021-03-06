# dusty
<h1> Dusty : A thread transaction based High Performance Rust Web Server </h1>

<img src="http://spurlock.io/src/DustyLayout.png" />

<h2>What is Dusty?</h2>
Dusty is a multi threaded web server written in RUST that implements two high performance thread pools. The HTTP parser thread pool receives incoming connections and sends requests generated by those connections to the requestHandler(gatherer) pool.The parser threads have a small buffer and start spawning gatherer threads before they even finish parsing. This can all happen due to dusty’s HTTP transaction system. When a parser thread reads a request it generates a request object which is then fed to the gatherer pool. Once a gatherer completes a transaction it sends the finished request back to the parser thread to be sent back over the socket. 
<br />
<br />
While dusty doesn’t wait until the end of the request to start calling gatherers it does wait until the beginning of the headers.Dusty implements a cache and in a multiget there are several files to gather from disk or from cache. If you know anything about computing performance you know the difference between reading from cache and disk can be a lifetime. Dusty takes this into account. It checks to see if the requested file is in the cache and then sorts the the files by size, which is also stored in the cache. If the file isn’t in the cache it is automatically given the value of a full usize to make sure it goes to the bottom of the sort. In Rust, usize’s are the size of a pointer: 64 bits on a 64 bit machine and 32 bits on a 32 bit machine. This all plays toward what Dusty is all about. Maximizing CPU utilization. If a request is blocked by I/O there are hundreds of other requests to route right this microsecond.
