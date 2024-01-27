import sys

def main():
    # sys.argv[0] is the script name itself
    # team 1 odds
    a_1 = int(sys.argv[1])
    b_1 = int(sys.argv[2])

    #team 2 odds
    a_2 = int(sys.argv[3])
    b_2 = int(sys.argv[4])

    bet_money = int(sys.argv[5])

    #strategy - find if we bet x on team 1 and 100-x on team 2, what is the range at which we win
    #Team 1 loses, then -X + a_2/b_2*(100-X) > 0 or 
    
    print(f"Bet anywhere from {(b_1*bet_money)/(a_1 + b_1)} to {(b_2*bet_money)/(a_2 + b_2)}")

if __name__ == "__main__":
    main()
