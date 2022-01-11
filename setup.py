from setuptools import setup
from Cython.Build import cythonize

setup(
    name='Board_Scorer',
    ext_modules=cythonize("Board_Scorer.pyx"),
    zip_safe=False,
)