========
Versions
========

Small tool for generate version-string from ``time.time()``.


Usage
=====

.. code:: bash

    $ ./versions.py
    p272or
    $ ./versions.py version-file
    p272oz
    $ cat version-file
    p272oz


That is all.


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


Future
======

Rewrite on RUST.
