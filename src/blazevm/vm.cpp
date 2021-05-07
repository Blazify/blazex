/*
   Copyright 2021 BlazifyOrg

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/
#include <iostream>
#include <string>
#include "vm.h"
using namespace std;

void VM()
{
    string name = getenv("bze_name");
    string content = getenv("bze_content");

    cout << "---Blaze Virtual Machine---" << endl
         << "Version: 0.0.1" << endl
         << "File: " << name << endl
         << "Content: " << content << endl;
}
