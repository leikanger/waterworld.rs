# WaterWorld.rs - a corrotion of karpathy's WaterWorld environment 

See Leikanger's [PhD thesis on Autonomous Navigation](https://hdl.handle.net/10037/25518) by neoRL for neural representation of Euclidean space for autonomous navigation.

See karpathy's [original webpage](http://cs.stanford.edu/people/karpathy/reinforcejs) for details, documentation and demos on the original idea.
See Python Pygame Learning environment implementation [here](http://pygame-learning-environment.readthedocs.io/en/latest/user/games/waterworld.html).

Please contact me if you're interested in autonomous navigation research: www.neoRL.net

# The environment
Five actions, NSEW, can be controled by keyboard WSAD/Up-Down-Right-Left. 
The RL environment adhere to conventions from OpenAI Gym.

The WaterWorld environment is used for POC and R&D on autonomous reactive navigation in Euclidean spaces.
The objetive of the agent to capture green creeps (+1) whilst avoiding red creeps (-1). 
After one creep has been encountered, a new is spawned with random location, speed vactor, and color.
All actors bounce when hitting the board boundary (wall).

The continouous Euclidean space with inertia is great for developing algorithms for autonomous navigation learning.
It should be run in real time, but can also be run at high-speed for DeepRL research (with GUI feature flag enabled)

# License

MIT.
