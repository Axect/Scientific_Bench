# Python Code for description
import numpy as np
import sys
import os
import shutil

def write():
    data_dir = "data/numpy"
    # if dir exists, delete it
    if os.path.exists(data_dir):
        shutil.rmtree(data_dir)
    os.mkdir(data_dir)
    m_vec = np.random.rand(100 * 500)
    m_vec[9301] = 0.1
    for i in range(100):
        dir = f"{data_dir}/run_{i:03d}/"
        os.mkdir(dir)
        for j in range(500):
            trial_dir = f"{dir}/trial_{j:04d}/"
            os.mkdir(trial_dir)
            m = m_vec[i * 500 + j]
            matrix = np.random.rand(500, 4)
            np.savez(f"{trial_dir}/data", m=m, matrix=matrix)

def update():
    data_dir = "data/numpy"
    for i in range(100):
        dir = f"{data_dir}/run_{i:03d}/"
        for j in range(500):
            data = np.load(f"{dir}/trial_{j:04d}/data.npz")
            if data["m"] == 0.1:
                matrix = np.zeros((500, 4))
                np.savez(f"{dir}/trial_{j:04d}/data", m=0.1, matrix=matrix)

def read():
    dir = "data/numpy"
    for i in range(100):
        for j in range(500):
            npz_dir = f"{dir}/run_{i:03d}/trial_{j:04d}/data.npz"
            data = np.load(npz_dir)
            if data["m"] == 0.1:
                print(f"id: {i:03d}, j: {j:04d}, m: {data['m']}")
                print(data["matrix"][0,:])

if __name__ == "__main__":
    args = sys.argv
    if args[1] == "write":
        write()
    elif args[1] == "update":
        update()
    elif args[1] == "read":
        read()
