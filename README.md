# Cartago
A ground-up implementation of the capabilities of the geos and proj libraries in ideomatic rust

The GEOS library is one of the primeir geoprocessing libraries and the foundation of numerous open-source
projects.  This is the case for the geos crate which provides rust bindings for the geos library.
While this makes for an expedient solution for rust projects requiring geoprocessing.

The CISA document, A Case for Memory Safe Roadmaps, recognizes that wrapping the API of packages written in languages known not to be memory safe is a good step towards memory safety, this comes with the additional provision that each API wrapper must also perform all necessary input sanitization prior to handing the call off to the underlying library.  While it is possible that the maintainers of the geos crate have made a good attempt to provide input validation, it would be nearly impossible to catch every posible failure condition.  The other consideration is that validating the inputs in runtime saps performance.

Cartago's purpose is to provide a memory safe implementation of geoprocessing algorithms from the ground up using rust with its unparalelled meory safety and performance.  

The main challenge is that both GEOS and PROJ involve a LOT of inheritance.  This means that a function by function port is not going to be a successful exercise.  Rather, the capabilities of these two libraries will need to be analyzed for traits, enumerations and generics.  An initial temptation would be to translate the PJ_COORD union with 12 coordnate variants to a rust enum.  While this would be a valid port option, it ultimately would make the source code very difficult to read and maintain because every calcuation that required working with coordinates would need to have a match clause to resolve each of the types.

