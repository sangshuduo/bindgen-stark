#include "wrapper.h"
#include <memory>

using std::pair;

extern "C" {

std::unique_ptr<pair<libstark::BairInstance, libstark::BairWitness>> wrap_bair() {
    return std::make_unique<pair<libstark::BairInstance, libstark::BairWitness>>(PCP_UTESTS::generate_valid_constraints());
}

}

