#ifndef _WRAPPER_H_
#define _WRAPPER_H_

#include "BairWitnessChecker_UTEST.hpp"

extern "C" {
    typedef std::pair<libstark::BairInstance, libstark::BairWitness> bair_t;
    typedef std::unique_ptr<bair_t> bair_ptr;

    bair_t wrap_bair();
}
#endif  // _WRAPPER_H_
