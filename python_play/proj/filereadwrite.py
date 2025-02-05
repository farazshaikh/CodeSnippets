if __name__ == "__main__":
    fd = open("/tmp/foo.txt", 'w+b')
    text = 'This is a really bad string'

    fd.write(text.encode('utf-8'))
    fd.close()

    print("reading the file");
    fd = open("/tmp/foo.txt", "r")
    p = fd.read();
    print(p)
