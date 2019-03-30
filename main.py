#/bin/python3
'''
Into the Breach.

'''

import logging
import tkinter as tk
from app.controllers import Controller

fn  ="openitb.log"
logging.basicConfig(filename= fn, filemode='w', level = logging.DEBUG, format= "%(levelname)s %(asctime)s %(funcName)s @%(lineno)d %(message)s")

if __name__ == "__main__":
    PARENT = tk.Tk()
    C = Controller(PARENT)
    C.run(debug_mode=False)
