import baseConfig from "../configurations/base.json" with { type: "json" };;

const buildConfiguration = () => {
  let configurations = baseConfig;

  for (const key in process.env) {
    if (key.startsWith("APP_")) {
        console.log(key);
        let keyLevel1 = "", keyLevel2 = "";
        let configKey = key.split("_")[1];

        if(key.includes("__")){
            keyLevel1 = configKey.split("__")[0];
            keyLevel2 = configKey.split("__")[1];
        }else{
            keyLevel1 = configKey;
        }

        if(keyLevel2){
            configurations[keyLevel1][keyLevel2] = process.env[key];
        }else{
            configurations[keyLevel1] = process.env[key];
        }
    }
  }

  return configurations;
};

export default buildConfiguration;
