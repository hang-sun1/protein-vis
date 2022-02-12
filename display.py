import sys
import os
import subprocess
import requests


def download(url):
    get_response = requests.get(url,stream=True)
    file_name  = url.split('/')[-1]
    with open(file_name, 'wb') as f:
        for chunk in get_response.iter_content(chunk_size=1024):
            if chunk: 
                f.write(chunk)

def extract(file_name):
    coords = []
    with open(file_name, 'r') as f:
        for line in f:
            line = [x for x in line.split(' ') if x != '']
            if line[0] == 'ATOM' and line[2] == 'CA':
                coords.append(f'{line[6]},{line[7]},{line[8]}')
    return coords

if __name__ == "__main__":
    url = f'https://files.rcsb.org/download/{sys.argv[1].upper()}.pdb'
    print(f'Downloading pdb assembly {sys.argv[1].upper()} from {url}')
    download(url)

    file_name = url.split('/')[-1]

    coords = extract(file_name)
    print(coords)

    start_dir = os.getcwd();
    os.chdir(f'{start_dir}/ray-tracing')
    subprocess.run(['cargo', 'build', '--release'])   
    os.chdir(start_dir)
    print('tracing rays')
    subprocess.run(['./ray-tracing/target/release/ray-tracing'] + coords)

    print('cleaning up')
    os.remove(url.split("/")[-1])
