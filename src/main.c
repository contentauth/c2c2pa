
#include <stdio.h>
#include <stdlib.h>

// This value needs to be large enough to hold the largest expected manifest
// The method will fail if the buffer is not large enough (ou could retry with a larger one to recover)
#define MANIFEST_SIZE 1024*20

// functions exported from the c2c2pa library
extern void verify_bytes(const char *format, const char *bytes, int length);
extern int sign_bytes(const char *format, const char *source_bytes, int length, char *dest_bytes, int dest_length);


int main(void)
{
   	FILE *file;
	char *buffer;
	int bufferSize;
	char *destBuffer;
	int destBufferSize;
	unsigned long fileLen;

	//Open file
	file = fopen("rsc/IMG_0003.jpg", "rb");
	if (!file)
	{
		fprintf(stderr, "Unable to open file");
		return 1;
	}
	
	//Get file length
	fseek(file, 0, SEEK_END);
	fileLen=ftell(file);
	fseek(file, 0, SEEK_SET);

	//Allocate memory
	bufferSize = fileLen+1;
	buffer= malloc(bufferSize);
	if (!buffer)
	{
		fprintf(stderr, "Memory error!");
        fclose(file);
		return 1;
	}

	//Read file contents into buffer
	fread(buffer, fileLen, 1, file);
	fclose(file);

	destBufferSize = fileLen+1+MANIFEST_SIZE;
	destBuffer= malloc(bufferSize+MANIFEST_SIZE);
	if (!destBuffer)
	{
		fprintf(stderr, "Memory error!");
        fclose(file);
		return 1;
	}

    int result = sign_bytes("jpeg", buffer, bufferSize, destBuffer, destBufferSize);
	if (result < 0)
	{
		fprintf(stderr, "Error %d signing manifest!", result);
		return 1;
	}
	
	// call the rust function and verify 
    verify_bytes("jpeg", destBuffer, result);

	file = fopen("target/with_claim.jpg", "wb");
	if (!file)
	{
		fprintf(stderr, "Unable to open output file");
		return 1;
	}
	fwrite(destBuffer, result, 1, file);
	fclose(file);

	free(buffer);
	free(destBuffer);

	return 0;
}