"""
Using Setuptools to both add lib to PYTHONPATH and specify console scripts.

Unfortunately conda develop can only do the former, so have to use this.
"""

from setuptools import find_packages, setup

setup(
    name="py_ans",
    version="0.0.1",
    packages=find_packages(),
    package_dir={"": "src"},
    include_package_data=True,
    entry_points={},
)
