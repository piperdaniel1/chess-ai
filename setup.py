from setuptools import setup
from Cython.Build import cythonize

# build with $python3 setup.py build_ext --inplace

setup(
    name='Board_Scorer',
    ext_modules=cythonize(["*.pyx"]),
    zip_safe=False,
)