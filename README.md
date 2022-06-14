# minecraft-pixel-art-generator

CLI to make pixel arts inside minecraft
This project was made in live on https://www.twitch.tv/willyandan

## Results
### Example 1
Input:   
![Example 1 input](https://user-images.githubusercontent.com/24833417/173480105-57affe68-6a38-435b-85e4-7d2e2064c21d.png)

Output:  
![Example 1 output](https://user-images.githubusercontent.com/24833417/173480199-364de49b-81c1-46ab-9b05-19d98dc2f7ab.png)

### Example 2
Input:  
![Example 2 input](https://user-images.githubusercontent.com/24833417/173481342-7c3c7851-57ee-48ef-aaa7-5ebbd78818a5.png)

Output:  
![Example 2 output](https://user-images.githubusercontent.com/24833417/173481305-b37d078a-f30e-426c-9ae1-01ac959df39e.png)



## How to run it
- Download the .exe https://github.com/willyandan/minecraft-pixel-art-generator/releases/download/v1.0.1/minecraft-pixel-art-generator.exe (Unfortunally it's only working for windows right now)
- Save it inside your .minecraft
- open a cmd and run it:
  ```
  "C:\Users\User\AppData\Roaming\.minecraft\minecraft-pixel-art-generator.exe" --name=example --world="World Name" --filename="C:\Users\User\Pictures\example.png"
  ```
  Or run it inside the .minecraft folder
  ```
  minecraft-pixel-art-generator.exe --name=example --world="World Name" --filename="C:\Users\User\Pictures\example.png"
  ```
  
  --name = the name that the function will receive
  --world = minecraft world's name (you can find it inside the 'saves' folder)
  --filename = path to the image
- after run the script you can run the command inside the minecraft to create it
  - /reload - to reload the datapack (can be util if you run with the game open)
  - /function pixelart:name - the script will create a command with the name argument E.g. /function pixelart:example 
  
## How to contribute
Feel free to open a pull request with the improvement, just create it in a branch called feat-feature-name or link it with a open issue
