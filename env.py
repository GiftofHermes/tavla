import numpy as np

class Board:
    def __init__(self):
        self.board = np.zeros(24,dtype=np.int8)
        self.board[0] = 2
        self.board[5] = -5
        self.board[7] = -3
        self.board[11] = 5
        self.board[12] = -5
        self.board[16] = 3
        self.board[18] = 5
        self.board[23] = -2

        self.turn = None

        self.white_hit = 0
        self.black_hit = 0

    def __str__(self):
        def draw_line():
            return '_'*25 + '\n'

        def draw_half(half_board, up, max):
            half = ''
            if up:
                iterator = range(max)
            else:
                iterator = range(max-1, -1, -1)
            for i in iterator:
                for j in range(12):
                    half += '|'
                    if abs(half_board[j]) > i:
                        half += '+' if np.sign(half_board[j]) == 1 else '-'
                    else:
                        half += ' '
                half += '|'
                half += '\n'
            half += draw_line()
            return half

        string = ''
        upper = np.flip(self.board[0:12]).copy()
        lower = self.board[12:].copy()
        max_checker_up = abs(upper).max()
        max_checker_low = abs(lower).max()
        string += draw_line()
        string += draw_half(upper, up=True, max=max_checker_up)
        string += draw_half(lower, up=False, max=max_checker_low)

        return string

    def check_collectable(self):
        if (self.turn == 'white') & (self.white_hit == 0) & ((self.board[0:18] > 0).sum() == 0):
            print((self.board[0:18] > 0).sum())
            return True
        if (self.turn == 'black') & (self.black_hit == 0) & ((self.board[7:] < 0).sum()) == 0:
            return True
        return False

    def valid_moves(self, dice):
        moves = []
        is_collectable = self.check_collectable()
        if self.turn == 'white':
            check = self.board > 0
        else:
            check = self.board < 0
        indices = np.argwhere(check).ravel()
        for index in indices:
            if self.check_valid_move(index, dice, is_collectable):
                moves.append([index, dice])
        return moves


    def check_valid_move(self, index, dice, is_collectable):
        if is_collectable:
            max_board = 24
            min_board = -1
        else:
            max_board = 23
            min_board = 0

        if dice > 6:
            return False
        if dice < 1:
            return False

        if self.turn == 'white':
            if index+dice >= max_board:
                return False
            if np.sign(self.board[index]) != 1:
                return False
            if self.board[index+dice] <= -2:
                return False
        elif self.turn == 'black':
            if index-dice <= min_board:
                return False
            if np.sign(self.board[index]) != -1:
                return False
            if self.board[index-dice] >= 2:
                return False
        return True


    def push(self, index, dice):
        is_collectable = self.check_collectable()
        is_valid_move = self.check_valid_move(index, dice, is_collectable)
        if not is_valid_move:
            raise Exception
        #change to enum
        if self.turn == 'white':
            if is_collectable & index+dice == 24:
                self.board[index] -=1
                self.white_collected +=1
            else:
                self.board[index] -= 1
                if self.board[index+dice] == -1:
                    self.black_hit += 1
                    self.board[index+dice] = +1
                else:
                    self.board[index+dice] +=1
        elif self.turn == 'black':
            if is_collectable & index-dice == -1:
                self.board[index] +=1
                self.black_collected +=1
            else:
                self.board[index] += 1
                if self.board[index-dice] == 1:
                    self.white_hit += 1
                    self.board[index-dice] = -1
                else:
                    self.board[index-dice] -=1
        else:
            raise ValueError
        return self


def main():
    board = Board()
    board.turn = 'black'
    print(board.valid_moves(1))
    print(board)
    #board.push(0, 6)
    #print(board)
    #board.push(0,4)
    #print(board)
    #board.turn = 'black'
    #board.push(23, 3)
    #print(board)
    #board.push(23, 2)
    #print(board)
if __name__ == '__main__':
    main()
