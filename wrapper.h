#include "BairWitness.hpp"
#include "BairWitnessChecker_UTEST.hpp"

std::pair<libstark::BairInstance,libstark::BairWitness> wrapper_generate_valid_constraints() {
    return PCP_UTESTS::generate_valid_constraints();
}
