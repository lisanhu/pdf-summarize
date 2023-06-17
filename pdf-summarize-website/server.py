from flask import Flask, request, jsonify
import requests
import os
import subprocess
import dotenv
import random

dotenv.load_dotenv()

app = Flask(__name__, static_url_path='')

@app.route('/')
def home():
    return app.send_static_file('index.html')

@app.route('/upload', methods=['POST'])
def upload_file():
    files = request.files.getlist('file')
    urls = request.form.getlist('url')
    result = ''

    for file in files:
        file.save(file.filename)
        result += f"{file.filename}\n===========\n"
        result += run_command(file.filename)
        result += "\n===========\n"
        os.remove(file.filename)

    random_string = ''.join(random.choices('abcdefghijklmnopqrstuvwxyz0123456789', k=10))
    i = 0
    for url in urls:
        if not url.startswith("http"):
            break
        r = requests.get(url, allow_redirects=True)
        filename = random_string + "_" + str(i) + ".pdf"
        open(filename, 'wb').write(r.content)
        result += f"{filename}\n===========\n"
        result += run_command(filename)
        result += "\n===========\n"
        os.remove(filename)
        i += 1

    return jsonify(result=result)

def run_command(filename):
    # Replace this with the actual command you want to run
    command = ['./pdf-summarize', filename]
    print(command)
    result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    print(result.stdout.decode('utf-8'))
    print(result.stderr.decode('utf-8'))
    return result.stdout.decode('utf-8')

if __name__ == '__main__':
    app.run(host="0.0.0.0", port=5000)
