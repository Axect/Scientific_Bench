# Python Code for description
import numpy as np
import sys
import os
import shutil
import struct
import re

# Convert float to bits
def float2bits(f):
    packed = struct.pack('>d', f)  # '>d' means big-endian double precision float
    bits = int.from_bytes(packed, byteorder='big')
    return bits

def bits2float(b):
    packed = b.to_bytes(8, byteorder='big')  # 8 bytes for double precision
    return struct.unpack('>d', packed)[0]

ROW = 500
COL = 4

def write():
    data_dir = "data/numpy"
    # if dir exists, delete it
    if os.path.exists(data_dir):
        shutil.rmtree(data_dir)
    os.mkdir(data_dir)

    m_pbh_vec = np.arange(1, 101) * 1e+15
    m_a_vec = np.arange(0.001, 0.1, 0.001)

    for m_pbh in m_pbh_vec:
        m_pbh_bits = float2bits(m_pbh)
        dir = f"{data_dir}/m_pbh_{m_pbh_bits}/"
        os.mkdir(dir)
        for m_a in m_a_vec:
            m_a_bits = float2bits(m_a)
            trial_dir = f"{dir}/m_a_{m_a_bits}/"
            os.mkdir(trial_dir)
            matrix = np.random.rand(ROW, COL)
            np.savez(f"{trial_dir}/data", matrix = matrix)

def update():
    data_dir = "data/numpy"
    # list all directories in data_dir
    for dir in os.listdir(data_dir):
        # get m_pbh via regex
        p = re.compile(r"m_pbh_(\d+)")
        m_pbh_bits = p.match(dir)
        m_pbh_bits = m_pbh_bits.group(1)
        m_pbh = bits2float(int(m_pbh_bits))

        run_dir = f"{data_dir}/{dir}"
        for trial_dir in os.listdir(run_dir):
            p = re.compile(r"m_a_(\d+)")
            m_a_bits = p.match(trial_dir)
            m_a_bits = m_a_bits.group(1)
            m_a = bits2float(int(m_a_bits))

            if m_a == 9e-2:
                npz_dir = f"{run_dir}/{trial_dir}/data.npz"
                data = np.load(npz_dir)
                print(f"m_pbh: {m_pbh}, m_a: {m_a}\nmatrix: {data['matrix'][0,:]}")
                new_matrix = np.zeros((ROW, COL))
                np.savez(npz_dir, matrix = new_matrix)

def read():
    dir = "data/numpy"
    for file in os.listdir(dir):
        p = re.compile(r"m_pbh_(\d+)")
        m_pbh_bits = p.match(file)
        m_pbh_bits = m_pbh_bits.group(1)
        m_pbh = bits2float(int(m_pbh_bits))

        run_dir = f"{dir}/{file}"
        for trial_dir in os.listdir(run_dir):
            p = re.compile(r"m_a_(\d+)")
            m_a_bits = p.match(trial_dir)
            m_a_bits = m_a_bits.group(1)
            m_a = bits2float(int(m_a_bits))

            if m_a == 9e-2:
                npz_dir = f"{run_dir}/{trial_dir}/data.npz"
                data = np.load(npz_dir)
                print(f"m_pbh: {m_pbh}, m_a: {m_a}\nmatrix: {data['matrix'][0,:]}")

if __name__ == "__main__":
    args = sys.argv
    if args[1] == "write":
        write()
    elif args[1] == "update":
        update()
    elif args[1] == "read":
        read()
