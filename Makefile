BIN = rwalld
GEN = rwall_clnt.c rwall_svc.c rwall_xdr.c rwall.h
RPCCOM = rpcgen
CFLAGS = -g -DDEBUG

all: $(BIN)

rwalld: rwall_server.o rwall_svc.o rwall_xdr.o
	$(CC) $(CFLAGS) -o $@ rwall_server.o rwall_svc.o rwall_xdr.o

rwall_server.o: rwall_server.c rwall.h
	$(CC) -c $(CFLAGS) rwall_server.c

$(GEN): rwall.x
	$(RPCCOM) -C rwall.x

clean cleanup:
	rm -f $(GEN) *.o $(BIN)
