#/bin/python3
'''
Into the Breach.

'''

import tkinter as tk
from app.controllers import Controller

if __name__ == "__main__":
    PARENT = tk.Tk()
    C = Controller(PARENT)
    C.run(debug_mode=False)
