#include "rwall.h"
#include "stdio.h"

void * wallproc_wall_1_svc(wrapstring *s, struct svc_req *rqstp __unused) {
	static void *dummy = NULL;
  static int msgno = 1;

  char newfile[200];
  sprintf(newfile, "motd%d.txt", msgno++);
  remove(newfile);
  rename("motd.txt", newfile);

  FILE *pfp;
  pfp = fopen("motd.txt", "w");
  if (pfp != NULL) {
    fprintf(pfp, "%s\n", *s);
    fclose(pfp);
  }
	return(&dummy);
}

