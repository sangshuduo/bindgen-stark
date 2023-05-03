#include "wrapper.h"

using std::pair;

extern "C" {

Bair *wrap_bair() {
    Bair *bair = (Bair *)malloc(sizeof(Bair));
    pair<libstark::BairInstance, libstark::BairWitness> retPair = PCP_UTESTS::generate_valid_constraints();
    bair->instance = &retPair.first;
    bair->witness = &retPair.second;
    return bair;
}

void free_bair(Bair *bair) {
    delete bair->instance;
    delete bair->witness;
    free(bair);
}

}
