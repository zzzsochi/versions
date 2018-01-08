========
Versions
========

Small tool for generate version-string from ``time.time()``.


Usage
=====

Simple
------

.. code:: bash

    $ ./versions.py
    p272or

Save to file
------------

.. code:: bash

    $ ./versions.py version-file
    p272oz
    $ cat version-file
    p272oz

Prefix and suffix
-----------------

.. code:: bash

    $ /versions.py --prefix=master- --suffix=-zzz
    master-p28nus-zzz

Shell variables
---------------

:VERSIONS_FILE: file for save
:VERSIONS_PREFIX: prefix
:VERSIONS_SUFFIX: suffix


Docker version
==============

.. code:: bash

    $ docker run --rm -t zzzsochi/versions
    p273wl


Plugin for drone.io
===================

.. code:: yaml

    pipeline:
      versions:
        image: zzzsochi/versions
        file: build/version
        prefix: master
        suffix: zzz


Future
======

Rewrite on RUST.
