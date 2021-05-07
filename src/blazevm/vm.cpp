#include <iostream>
#include <string>
#include "vm.h"
using namespace std;

void VM()
{
    string name = getenv("bze_name");
    string content = getenv("bze_content");
    cout << "BlazeVM\nVersion: 0.0.1\nDeveloped by the BlazifyOrg\n"
         << "Executable: " << name << endl
         << "Content: " << content << endl;
}
