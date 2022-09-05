use anyhow::anyhow;

mod omega_factors;

pub use omega_factors::OmegaFactors;

use crate::{
    constants::{C_M_PER_S, DEFAULT_NEUTRINO_MASSES, DEFAULT_N_EFF, ONE, ZERO},
    eV, units,
    units::{PositiveFloat, Seconds},
    DimensionlessPositiveFloat, HInvKmPerSecPerMpc, Kelvin, KmPerSecPerMpc, Mpc, Redshift,
};

/// Represents an FLRW cosmology.
///
/// This represents an homogenous and isotropic cosmology based
/// on the FLRW (Friedmann-Lemaitre-Robertson-Walker) metric.
pub struct FLRWCosmology {
    /// A descriptive name.
    pub name: Option<String>,
    /// Literature reference.
    pub reference: Option<String>,

    /// Hubble constant at `z=0` (km/(s/Mpc)).
    pub H_0: KmPerSecPerMpc,

    /// Omega factors for this cosmology.
    pub omega: OmegaFactors,

    /// Temperature of the CMB at `z=0`.
    pub T_CMB0: Kelvin,
    /// Number of effective neutrino species.
    pub N_eff: DimensionlessPositiveFloat,
    /// Mass of neutrino species in eV.
    pub m_nu: Vec<eV>,
}

impl FLRWCosmology {
    /// Instantiate a new FLRW cosmology.
    pub fn new(
        name: Option<String>,
        reference: Option<String>,
        H_0: f32,
        omega: OmegaFactors,
        T_CMB0: Option<Kelvin>,
        N_eff: Option<DimensionlessPositiveFloat>,
        m_nu: Option<Vec<eV>>,
    ) -> Result<Self, anyhow::Error> {
        if N_eff.unwrap_or(*DEFAULT_N_EFF).floor()
            != m_nu
                .clone()
                .unwrap_or_else(|| DEFAULT_NEUTRINO_MASSES.to_vec())
                .len() as f32
        {
            return Err(anyhow!(
                "number of neutrino masses must match the number of effective neutrino species"
            ));
        }

        Ok(Self {
            name,
            reference,
            H_0,
            omega,
            T_CMB0: T_CMB0.unwrap_or(*ZERO),
            N_eff: N_eff.unwrap_or(*DEFAULT_N_EFF),
            m_nu: m_nu.unwrap_or_else(|| DEFAULT_NEUTRINO_MASSES.to_vec()),
        })
    }

    pub fn E(&self, z: Redshift) -> Mpc {
        PositiveFloat(
            (self.omega_m0().0 * (1. + z).powf(3.)
                + self.omega_k0().0 * (1. + z).powf(2.)
                + self.omega_de0().0)
                .sqrt(),
        )
    }

    /// Hubble expansion rate (km/s/Mpc) at redshift z.
    pub fn H(&self, z: Redshift) -> KmPerSecPerMpc {
        self.H_0 * self.E(z).0
    }

    /// Scale factor at redshift z.
    pub fn scale_factor(&self, z: Redshift) -> DimensionlessPositiveFloat {
        PositiveFloat(1.0 / (z + 1.0))
    }

    /// Dimensionless hubble parameter h where 100 km/s/Mpc * h = H0
    pub fn little_h(&self) -> DimensionlessPositiveFloat {
        DimensionlessPositiveFloat::new(self.H_0 / 100.0).unwrap()
    }

    /// Hubble time: Inverse of the Hubble constant H_0
    pub fn hubble_time(&self) -> Seconds {
        // H_0 units are km/s/Mpc so we need to convert Mpc to km
        // such that the distance units cancel
        PositiveFloat(1. / self.H_0 * units::MPC_TO_KILOMETERS)
    }

    /// Hubble distance in Mpc: $D_H = c / H_0$.
    pub fn hubble_distance(&self) -> KmPerSecPerMpc {
        // Factor of 1000 to convert c in m/s to c in km/s so that
        // the units cancel.
        *C_M_PER_S / (self.H_0 * 1000.)
    }

    /// Hubble distance in h^{-1} Mpc.
    pub fn hubble_distance_little_h(&self) -> HInvKmPerSecPerMpc {
        *C_M_PER_S / (1.0e5)
    }

    /// Critical density at `z=0`.
    pub fn critical_density0(&self) -> DimensionlessPositiveFloat {
        //TODO
        PositiveFloat::new(0.0).unwrap()
    }

    /// Dimensionless photon density (density/critical density) at `z=0`.
    pub fn omega_gamma0(&self) -> DimensionlessPositiveFloat {
        // TODO
        PositiveFloat::new(0.0).unwrap()
    }

    /// Dimensionless neutrino density (density/critical density) at `z=0`
    pub fn omega_nu0(&self) -> DimensionlessPositiveFloat {
        // TODO
        PositiveFloat::new(0.0).unwrap()
    }

    /// Dimensionless dark matter density (density/critical density) at `z=0`
    pub fn omega_dm0(&self) -> DimensionlessPositiveFloat {
        self.omega.omega_dark_matter_density_0()
    }

    /// Dimensionless effective curvature density (density/critical density) at `z=0`
    pub fn omega_k0(&self) -> DimensionlessPositiveFloat {
        // TODO
        PositiveFloat::new(0.0).unwrap()
    }

    /// Dimensionless matter density (density/critical density) at `z=0`
    pub fn omega_m0(&self) -> DimensionlessPositiveFloat {
        self.omega.Omega_M0
    }

    /// Dimensionless dark energy density (density/critical density) at `z=0`
    pub fn omega_de0(&self) -> DimensionlessPositiveFloat {
        self.omega.Omega_DE0
    }

    /// Dimensionless total density (density/critical density) at `z=0`.
    pub fn omega_tot0(&self) -> DimensionlessPositiveFloat {
        self.omega_m0()
            + self.omega_gamma0()
            + self.omega_nu0()
            + self.omega_de0()
            + self.omega_k0()
    }

    /// Whether this cosmology is spatially flat
    pub fn is_flat(&self) -> bool {
        self.omega_k0() == *ZERO && self.omega_tot0() == *ONE
    }

    pub fn neutrino_temperature(&self) {
        todo!()
    }
}
