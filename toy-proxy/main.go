package main

import (
	"fmt"
	"io"
	"net"
	"time"
)

const ioTimeout = time.Second * 10

// main ...
func main() {
	listener, err := net.Listen("tcp4", "0.0.0.0:1337")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer listener.Close()

	for {

		downstreamConn, err := listener.Accept()
		if err != nil {
			fmt.Print(err)
			continue
		}

		go func(downstreamConn net.Conn) {
			// upstreamLink and downstreamLink are created per connection made to 1337
			upstreamLink := &link{
				ch : make(chan []byte),
				latency: time.Second,
			}

			downstreamLink := &link{
				ch : make(chan []byte),
				latency: time.Second,
			}
			
			
			defer downstreamConn.Close()
			upstreamConn , err := net.Dial("tcp4", "localhost:8080")
			if err != nil {
				fmt.Println(err)
				return
			}
			defer upstreamConn.Close()
			// not production ready
			// to figure out ways to manage the same for sql connection 
			deadline := time.Now().Add(ioTimeout);
			downstreamConn.SetDeadline(deadline)
			upstreamConn.SetDeadline(deadline)

			// client -> downstreamConn -> upstreamLink -> upstreamConn -> server
			go io.Copy(upstreamLink, downstreamConn)
			go io.Copy(upstreamConn, upstreamLink)
			
			// server -> upsreamConn -< downstreamLink -> downstreamConn <- client

			go io.Copy(downstreamLink, upstreamConn)
			io.Copy(downstreamConn, downstreamLink)

		}(downstreamConn)
	}
}


type link struct  {
	ch chan []byte 
	latency time.Duration
}


// Read ...
func (l *link) Read(b []byte) (int ,  error) {
	select{
		
		case data := <- l.ch:
		time.Sleep(l.latency)
		copy(b, data)
		return len(data) , nil
		
		case <- time.After(ioTimeout):
		return  0, io.EOF
	}
}

// Write ...
func (l *link) Write(b []byte) (int, error) {
	l.ch <- b
	return len(b), nil
}
