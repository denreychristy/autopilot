# AutoPilot - Main

# ================================================================================================ #
# Imports

from rust import Currency # type: ignore
from rust import NonPlayerCharacter as NPC # type: ignore

# ================================================================================================ #

class AutoPilot:
	def __init__(self):
		currency = Currency(
			"dollar",
			"dollars",
			0
		)

		npc = NPC()
		print(npc.get_id(), npc.get_currency())
	
	# ================================================== #
	# Class Methods

	# ================================================== #
	# Dunder Methods

	# ================================================== #
	# Property Methods

	# ================================================== #
	# Set Methods

	# ================================================== #
	# Other Methods

# ================================================================================================ #

if __name__ == '__main__':
	AutoPilot()