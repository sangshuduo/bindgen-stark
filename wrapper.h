#ifndef _WRAPPER_H_
#define _WRAPPER_H_

#include "BairWitness.hpp"
#include "BairWitnessChecker_UTEST.hpp"

typedef struct wrap_bair_s {
    libstark::BairInstance *instance;
    libstark::BairWitness *witness;
} Bair;

extern "C" {
    Bair *wrap_bair();
}
#endif  // _WRAPPER_H_
