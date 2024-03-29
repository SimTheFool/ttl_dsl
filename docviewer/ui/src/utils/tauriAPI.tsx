"use client";

import { useRenderingContext } from "@/components/controls/RenderingContext";
import { Box } from "@radix-ui/themes";
import { invoke } from "@tauri-apps/api/tauri";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import ClipLoader from "react-spinners/ClipLoader";

const getData = async <T,>(
  dataFile: string,
  resolutionDir: string,
  parser: (data: unknown) => T
) => {
  const [json, images] = (await invoke("get_template_data", {
    dataFile: dataFile,
    resolutionDir: resolutionDir,
  }).catch((e: any) => {
    console.error(`Invoke error: ${e}`);
    throw new Error(`Invoke error: ${e}`);
  })) as [json: unknown, images: Record<string, string>];

  const imagesWithAssetLinks = Object.fromEntries(
    Object.entries(images).map(([key, value]) => [key, convertFileSrc(value)])
  );

  let parsedData: T;
  try {
    parsedData = parser(json);
  } catch (e) {
    console.error(`Parse data error: ${e}`);
    throw new Error(`Parse data error: ${e}`);
  }

  return [parsedData, imagesWithAssetLinks] as const;
};

type RenderDataProps<T> = {
  Child: React.FC<{ data: T; images: Record<string, string> }>;
  parser: (data: unknown) => T;
};
export const RenderData = <T,>({ Child, parser }: RenderDataProps<T>) => {
  const { dataFile, resolutionDir } = useRenderingContext();
  const [json, setJson] = useState<T>();
  const [images, setImages] = useState<Record<string, string>>({});
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string>();
  console.log("error !!", error);

  useEffect(() => {
    if (!dataFile || !resolutionDir) {
      return;
    }
    setIsLoading(true);
    setError(undefined);
    getData(dataFile, resolutionDir, parser)
      .then(([json, images]) => {
        setJson(json);
        setImages(images);
      })
      .catch((e) => {
        setError(e.message);
        console.error(e);
      })
      .finally(() => {
        setIsLoading(false);
      });
  }, [dataFile, resolutionDir, setIsLoading]);

  if (isLoading || error) {
    return (
      <Box
        style={{
          width: "100%",
          height: "100vh",
          position: "relative",
        }}
      >
        <Box
          style={{
            position: "absolute",
            top: "50%",
            left: "50%",
            transform: "translate(-50%, -50%)",
          }}
        >
          <ClipLoader loading={isLoading} size={150} />
          {error && <pre>{error}</pre>}
        </Box>
      </Box>
    );
  }

  if (!json) {
    return null;
  }

  return <Child data={json} images={images} />;
};

const fake =
  '{"name":"Shrimp","tags":{"HnHu23uwtH":"humain","ffaREphb5z":"technorigger","yFgZWf3hjj":"13 ans"},"knowledges":{"LiTFK85K2W":"Séries tridéos","hm4kr5vQLm":"Drônes","nPrZoOE3PX":"Jeux tridéos"},"identities":{"ThsPAOue0w":{"name":"Laurence Guinvite","price":10000,"nuyens":1710,"quality":4,"lifestyle":{"name":"squatteur","price":100},"licences":{"V4f4789ug9":{"name":"Concierge de chantier","price":800,"quality":4}}},"Wky65v2oZD":{"contacts":{"UuG7ILFt1t":{"name":"D-Boss","loyalty":4,"connection":4,"description":"Decker fan de complot"},"cmRdVaGO51":{"name":"Terrance","loyalty":3,"connection":2,"description":"Ouvrier de casse militaire d\'ARES"}}}},"stats":{"con":2,"con_mod":0,"agi":4,"agi_mod":0,"rea":2,"rea_mod":0,"for":1,"for_mod":0,"vol":4,"vol_mod":0,"log":6,"log_mod":0,"int":4,"int_mod":0,"cha":2,"cha_mod":0,"ess":6,"edge":4,"resist_phy":{"value":2,"metas":["con"]},"resist_ment":{"value":4,"metas":["vol"]},"def_phy":{"value":6,"metas":["rea","int"]},"def_ment":{"value":8,"metas":["vol","int"]},"init_dice":1,"init_base":{"value":6,"metas":["rea","int"]},"action_maj":1,"action_min":2,"hit_phy":9,"hit_stun":10,"hit_over":2,"heal":{"value":6,"metas":["con","vol"]},"resist_drain":{"value":3,"metas":["vol"]},"res":7,"submersion":1,"firewall":4,"traitement":6,"corruption":4,"attaque":2},"skills":{"athlétisme":{"score":3},"combat_rapproché":{"score":1},"furtivité":{"score":1},"ingénierie":{"score":6,"specializations":{"Jm6OOHrwaX":"Artillerie"}},"perception":{"score":1},"pilotage":{"score":6,"specializations":{"LcHF1i2DQp":"au sol"}},"technomancie":{"score":6,"specializations":{"hcw41sxYYd":"Compilation"},"masterizations":{"9gXp8tqjL1":"Inscription"}},"électronique":{"score":4}},"traits":{"Peur_du_pilotage_automatique":{"description":"__D3__ lorsque vous laissez vos machines en pilote automatique."},"Réseau_vivant":{"description":"Utlisez votre persona incarné comme noeud PAN."},"ami_des_sprites":{"description":"__A1__ lorsque vous compilez ou inscrivez un sprite machine."},"bricoleur_prévoyant":{"description":"__A1__ lorsque vous utilisez une machine que vous avez bricolé."},"paralysie_du_combat":{"description":"Au premier round, vous ne pouvez pas vous déplacer et vous jouez en dernier."},"rhinite_chronique":{"description":"Vous éternuez souvent. __D1__ lors des tests de discrétion."}},"powers":{"extraction_simsens":{"test":"Elec+RES","duration":"3h","description":"Décompilez un sprite pour créer des données simsens. Pour chaque __SN__, ajoutez --+1 qualité-- du fichier. Rajoutez __A1__ par palier d\'ancienneté du sprite (3 jours / 10 jours / 20 jours)."},"lecture_simsens":{"description":"Vous pouvez lire un fichier de donnée simsens."},"middleware":{"test":"Elec+RES","major":1,"minor":0,"maintained":true,"description":"Mobilisez un sprite pour créer un stream sensoriel interconnectant vos appareils. Percevez alors votre environnement sous plusieurs perspectives et pilotez facilement plusieurs machines. Pour chaque --Puiss.-- du sprite, rajoutez une --1h-- à l\'effet ou 1__RD__ à vos --tests de perception--. A la fin de l\'effet, le sprite est décompilé."}},"actions_common":{"accélérer":{"major":0,"minor":1,"description":"Accélérez un véhicule ou un drône. (une fois par tour)"},"ajuster":{"major":0,"minor":1,"description":"Gagner 1__RD__ pour votre attaque suivante"},"attaque_cruelle":{"major":1,"minor":0,"description":"Vos __SN__ permettent d\'infliger un effet de statut déterminé avec le MJ. Vous subissez __D2__ sur votre attaque."},"attaque_handicapante":{"major":1,"minor":0,"description":"Vous attaquez mais n\'infligez que les dégats de base de l\'arme. Vos __SN__ permettent d\'infliger un effet de statut déterminé avec le MJ."},"attaque_multiple":{"major":1,"minor":0,"description":"Faites plusieurs attaques sur une ou plusieurs cibles, en répartissant votre test équitablement entre toutes. Vous pouvez subir __D3__ pour utilisez votre réserve entière sur chaque cible."},"bloquer":{"major":0,"minor":1,"reaction":true,"description":"Ajoutez --Combat rapproché-- à votre --test de défense-- pour bloquer un coup."},"changer_de_mode":{"major":0,"minor":1,"description":"Changer d\'accès matriciel (RA/RV), de mode de tir, ou de controle d\'appareil"},"charger_attaquer":{"major":1,"minor":1,"description":"Faites votre --déplacement-- (au moins 5 mètres) ainsi qu\'un --test d\'attaque--. Ajoutez le déplacement restant à vos __SN__, ou retirez une partie de vos __SN__ pour vous déplacer plus loin, puis attaquez."},"charger_repousser":{"major":1,"minor":1,"description":"Faites votre --déplacement-- (au moins 5 mètres) ainsi qu\'un --test d\'attaque--. Ajoutez le déplacement restant à vos __SN__, ou retirez une partie de vos __SN__ pour vous déplacer plus loin, puis repoussez la cible de __SN__ (max FOR)."},"commander":{"major":0,"minor":1,"description":"Envoyer un message ou un ordre à une machine, à un drône, à un sprite ou à une invocation."},"défense_totale":{"major":1,"minor":0,"reaction":true,"description":"Ajouter votre --VOL-- à tous vos tests de défense jusqu\'à votre prochain tour."},"escalader":{"test":"Athl+FOR","major":1,"minor":0,"description":"Vous pouvez grimper --1m-- par __SN__."},"esquiver":{"major":0,"minor":1,"reaction":true,"description":"Ajouter votre --Athlétisme-- à votre --test de défense--."},"feinte":{"test":"Escro+CHA","major":0,"minor":1,"description":"Distrayez la cible. Tout le monde gagne __A1__ à ses action contre elle."},"intercepter":{"major":1,"minor":1,"reaction":true,"description":"Vous pouvez vous déplacer de --REA-- mètres pour attaquer une cible à portée. Si vos __SN__ sont supérieurs à sa CON, elle est interrompue."},"intimider":{"test":"Infl+CHA","major":1,"minor":0,"description":"Vous infligez Effrayé à la cible."},"lutter":{"test":"Athl+FOR","major":1,"minor":0,"description":"Maîtrisez un adversaire. En lutte, le défenseur se défend avec --CON+FOR--, et l\'attaquant attaque avec --Combat rapproché--."},"manœuvrer":{"test":"Pilot+INT","major":1,"minor":0,"description":"Manoeuvrez un véhicule ou un drône."},"mettre_la_gomme":{"major":1,"minor":0,"description":"Doublez l\'accélération d\'un véhicule ou d\'un drône. (une fois par tour)"},"parcourir":{"test":"Athl+FOR","major":2,"minor":0,"description":"Vous pouvez vous déplacer et escalader de --15m + 2 x--__SN__. Les obstacles (<1,5m) et les sauts (<2m) ne comptent pas."},"protéger":{"major":0,"minor":1,"reaction":true,"description":"Vous pouvez vous déplacer de --REA-- mètres pour prendre le coup à la place d\'une cible."},"risposter":{"major":0,"minor":1,"reaction":true,"description":"Subissez __D1__ sur votre --test de défense--. Si vous réussissez le test, vous ripostez et infligez __SN__ __DOM__ à votre attaquant."},"se_coucher":{"major":0,"minor":1,"description":null},"se_déplacer":{"major":0,"minor":1,"description":"Vous vous déplacez de 10m, une fois par tour."},"se_jeter_à_terre":{"major":0,"minor":1,"reaction":true,"description":"Vous obtenez 4__RD__ à votre --test de défense--, mais vous tombez --à terre--."},"se_mettre_à_couvert":{"major":0,"minor":1,"description":null},"se_relever":{"major":0,"minor":1,"description":null},"sprinter":{"test":"Athl+FOR","major":1,"minor":0,"description":"Vous pouvez vous déplacer de --15m +-- __SN__ et sauter les petits obstacles (<1m)."},"évitez_un_explosif":{"test":"Athl+REA","major":0,"minor":1,"reaction":true,"description":"Evitez le souffle d\'une explosion. Vous vous déplacez d\'1m par __SN__. Au delà de 2m, vous tombez --à terre--."}},"companions":{"Sprite_machine":{"name":"Sprite machine","stats_primary":{"major":1,"minor":4,"hit_formula":"8 + P / 2"},"stats_secondary":{"attaque":1,"corruption":3,"firewall":0,"traitement":2},"skills":{"WDqugJIwSZ":"Ingenierie","siglZtDQ4k":"Pilotage","yqu8tIQJ6V":"Electronique"},"actions":{"diagnostique":{"major":1,"minor":0,"maintained":true,"description":"Opère un monitoring de l\'appareil investi. Vous avez __A1__ lorsque vous utilisez l\'appareil."},"investir_un_appareil":{"major":1,"minor":0,"maintained":false,"description":"Le sprite occupe un appareil quelque soit le niveau d\'accréditation."},"stabilisation":{"major":1,"minor":0,"maintained":true,"description":"Surveille les dysfonctionnments de l\'appareil investi. Vous pouvez retirer jusqu\'à --Serv.-- echecs à vos tests avec cet appareil."}}}},"small_inventory":{"Charge_medikit":{"name":"Charge medikit","price":180,"price_unit":20,"quantity":9,"quality":1,"description":"","status":"free","concealment":5},"Chargeur_crockett":{"name":"Recharges balles","price":500,"price_unit":250,"quantity":2,"quality":1,"status":"free","concealment":1},"Chargeur_yamaha":{"name":"Recharges taser","price":60,"price_unit":30,"quantity":2,"quality":1,"status":"free","concealment":3},"Corde_standard":{"name":"Cordes standard","price":22,"price_unit":1,"quantity":22,"quality":1,"description":"Corde de 60m qui peut soutenir 400 Kg.","status":"free","concealment":3},"Dérivateur_de_données":{"name":"Dérivateur de données","price":300,"price_unit":300,"quantity":1,"quality":1,"description":"Copie les données transitant sur n\'importe quel cable de données et de fournit une connexion sans fil à un système filaire.","status":"free","concealment":4},"Kit_de_reparation":{"name":"Kit de réparation","price":1500,"price_unit":500,"quantity":3,"quality":1,"description":"","status":"free","concealment":3},"Medikit":{"name":"Medikit","price":1000,"price_unit":1000,"quantity":1,"quality":4,"description":"Soigne les __DOM__ récents (<1h). Utilise la compétence --Biotech-- ou l\'--indice-- de l\'appareil.","status":"free","concealment":3},"Patch_stim":{"name":"Patch stim","price":2400,"price_unit":400,"quantity":6,"quality":4,"description":"Soigne X __DOM__ étourdissants, pendant X x 10min. La cible subit X + 1 __DOM__ étourdissants.","status":"free","concealment":6},"Scanner_de_proximité":{"name":"Scanner de proximité","price":200,"price_unit":200,"quantity":1,"quality":1,"description":"Scanne les appareils sans fil dans un rayon de 20m.","status":"free","concealment":4}},"big_inventory":{"Bras_mécanique":{"name":"Bras mécanique","manufacturer":null,"price":1000,"price_unit":1000,"quantity":1,"quality":1,"description":"Adaptée pour les drônes.","status":"free","concealment":2},"Crawler":{"name":"Crawler","manufacturer":"Aztechnology","price":14500,"price_unit":14500,"quantity":1,"quality":1,"description":"Moyen arthropode mécanique","status":"free","concealment":2,"stats_primary":{"hit":11},"stats_secondary":{"armor":2,"autopilot":2,"maniability":3,"resistance":6,"speed_max":30,"speed_step":8},"slots":{"7dKW7WAYsd":{"name":"Chasse","size":"S"},"SmtjwnQZ8X":{"name":"Chasse","size":"S"}}},"Crockett":{"name":"Crockett","manufacturer":"Cavalier Arms","price":11350,"price_unit":11350,"quantity":1,"quality":1,"description":"Fusil d\'assaut polyvalentLa guncam permet de calculer la trajectoire des balles.Adaptée pour les drônes.Le canon a été allongé pour une meilleure précision à distance.Modifiée pour le démontage.","status":"illegal","concealment":0,"ranges":{"contact":{"label":0,"base":-2},"near":{"label":50,"base":0},"short":{"base":1},"mid":{"label":100,"base":2},"far":{"label":750,"base":1}},"actions":{"Tir":{"major":1,"minor":0,"damage":5,"ammo":1,"ranges":{"contact":-2,"near":0,"short":1,"mid":2,"far":1}},"Tir_rafale":{"major":1,"minor":0,"damage":7,"ammo":4,"ranges":{"contact":-3,"near":-1,"short":0,"mid":1,"far":0}},"Tir_semi_auto":{"major":1,"minor":0,"damage":6,"ammo":2,"ranges":{"contact":-3,"near":-1,"short":1,"mid":2,"far":1}},"démonter":{"major":2,"minor":0,"description":"Dissimulation +2 lorsque démontée."},"guncam":{"major":0,"minor":1,"description":"Vous ignorez les désavantages pour votre prochain tir à distance."},"recharger":{"major":1,"minor":0,"ammo_gauge":250}}},"Grappin":{"name":"Grappin","manufacturer":null,"price":350,"price_unit":350,"quantity":1,"quality":1,"description":"Adaptée pour les drônes.","status":"free","concealment":2,"ranges":{"contact":{"label":0,"base":-2},"near":{"label":20,"base":0},"short":{"base":0},"mid":{"base":0},"far":{"base":0}},"actions":{"Tir":{"major":1,"minor":0,"damage":1,"ammo":1,"ranges":{"contact":-2,"near":0,"short":0,"mid":0,"far":0}},"recharger":{"major":1,"minor":0,"ammo_gauge":10}}},"Invisi_Shield":{"name":"Invisi Shield","manufacturer":"Securetech","price":5400,"price_unit":5400,"quantity":1,"quality":1,"description":"Revêtement protecteur camouflé. Si vous ne subissez qu\'un __DOM__, vous n\'en subissez pas.","status":"free","concealment":4},"Kanmushi":{"name":"Kanmushi","manufacturer":"Shiawase","price":900,"price_unit":450,"quantity":2,"quality":1,"description":"Petit arthropode mécanique","status":"free","concealment":6,"stats_primary":{"hit":8},"stats_secondary":{"armor":0,"autopilot":2,"maniability":2,"resistance":0,"speed_max":15,"speed_step":4}},"Micro_et_caméra":{"name":"Micro_et_caméra","price":100,"price_unit":100,"quantity":1,"quality":1,"status":"free","concealment":6},"Packmule":{"name":"Packmule","manufacturer":"Ares","price":16000,"price_unit":16000,"quantity":1,"quality":1,"description":"Grand arthropode mécanique","status":"free","concealment":0,"stats_primary":{"hit":12},"stats_secondary":{"armor":10,"autopilot":2,"maniability":4,"resistance":8,"speed_max":30,"speed_step":6},"slots":{"NQdTQ2HciH":{"name":"Chasse","size":"S"},"dki0j0Obow":{"name":"Cache","size":"M","concealment":3},"vM8nLfVGqW":{"name":"Chasse","size":"S"}}},"Pulsar":{"name":"Pulsar","manufacturer":"Yamaha","price":350,"price_unit":350,"quantity":1,"quality":1,"description":"Taser.Adaptée pour les drônes.","status":"free","concealment":3,"ranges":{"contact":{"label":0,"base":0},"near":{"label":20,"base":0},"short":{"base":0},"mid":{"base":0},"far":{"base":0}},"actions":{"Tir":{"major":1,"minor":0,"damage":4,"ammo":1,"ranges":{"contact":0,"near":0,"short":0,"mid":0,"far":0}},"Tir_semi_auto":{"major":1,"minor":0,"damage":5,"ammo":2,"ranges":{"contact":-1,"near":-1,"short":-1,"mid":-1,"far":-1}},"recharger":{"major":1,"minor":0,"ammo_gauge":30}}},"Sac":{"name":"Sac","price":0,"price_unit":0,"quantity":1,"quality":1,"status":"free","concealment":0,"slots":{"HkSg8TFupL":{"size":"XL"}}},"Tenue_urbaine":{"name":"Tenue urbaine","price":550,"price_unit":550,"quantity":1,"quality":1,"description":"Confère __A1__ pour résister aux entraves.","status":"free","concealment":0,"actions":{"utiliser_poche_secrète":{"major":1,"minor":0}},"slots":{"PC547NQSh7":{"name":"Poche secrète","size":"S","concealment":5},"WUU5L4rw5g":{"name":"Poche secrète","size":"S","concealment":5},"yAkA1Nr0VC":{"name":"Poche","size":"S"}}},"clef_cyber_fictionnelle":{"name":"Clef anglaise robotique cyber fictionnelle","manufacturer":null,"price":725,"price_unit":725,"quantity":1,"quality":1,"description":"Une réplique de l\'arme emblématique de la série Technotronix. Le revêtement électrique rechargeable permet d\'électrifier la cible. Le nanogénérateur à électricité statique permet de toucher la cible même si on rate son coup.","status":"free","concealment":4,"ranges":{"contact":{"base":0},"near":{"base":0},"short":{"base":1},"mid":{"base":1},"far":{"base":0}},"actions":{"activer":{"major":0,"minor":1,"description":"+1 charges toutes les 15min. 1 __DOM__ irréductible et le statut --Electrocuté--, même si vous ratez.","gauge":4},"attaquer":{"major":1,"minor":0,"damage":3,"ranges":{"contact":0,"near":0,"short":0,"mid":0,"far":0}}}}}}';
