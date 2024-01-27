#include <iostream>
#include <vector>
#include <string>

using namespace std;

int main()
{
    //vector<string> msg {"Hello", "C++", "World", "from", "VS Code", "and the C++ extension!"};
        
    //for (const string& word : msg)
    //{
    //    cout << word << " ";
    //}
    cout<<"We will develop a hedging strategy to win any game"<<endl;
    int a_1 = 0; int b_1 = 0; int a_2 = 0; int b_2 = 0; int bet_money = 0;

    cin>>a_1>>b_1>>a_2>>b_2>>bet_money;

    cout<<"Feel free to bet from "<<(b_1*bet_money)/(a_1 + b_1)<<" to "<<(b_2*bet_money)/(a_2 + b_2)<<endl;
}