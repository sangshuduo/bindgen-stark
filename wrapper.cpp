#include "wrapper.h"
#include <memory>
#include <iostream>

extern "C" {

bair_t wrap_bair() {
    auto bair = PCP_UTESTS::generate_valid_constraints();
    std::cout << "Instance details: \n"
            << "- vector length: " << bair.first.vectorsLen() << std::endl
            << "- domain size: " << bair.first.domainSize() << std::endl
            << "- constraints assignment size: "
            << bair.first.constraintsAssignment().constraints().size()
            << std::endl
            << std::endl;
    return bair;
}

}

