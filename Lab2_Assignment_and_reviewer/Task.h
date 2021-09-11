//
// Created by Ivan on 07.09.2021.
//

#ifndef LAB2_ASSIGNMENT_AND_REVIEWER_TASK_H
#define LAB2_ASSIGNMENT_AND_REVIEWER_TASK_H

#include <iostream>
#include <string>

using namespace std;

class Task {
private:
    int id{};
    string name;
    int result{};
    static int total_id;

public:
    // Constructor's and destructor


    explicit Task (const string &new_name = "none");

    ~Task ();

    // Getters and setters
    int get_result () const;

//
//    static int get_total_id () ;
//
//    static void set_total_id(int add_total);
//
    void set_name (string new_name);

    friend void show_info (const Task &task_to_show);

    friend class Verifier;


};


#endif //LAB2_ASSIGNMENT_AND_REVIEWER_TASK_H
