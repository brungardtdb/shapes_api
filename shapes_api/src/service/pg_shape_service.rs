use crate::{app_state::AppState, service::ShapeService};
use shape_repositories::pg_repositories::*;
use shapes::aisc_shapes::ShapeRepository;
use shapes::aisc_shapes::*;

/// The service layer for the applicatoin
pub struct PGShapeService<AppState> {
    /// The app state for the application
    pub state: AppState,
}

impl
    PGShapeService<
        AppState<
            AngleRepository,
            CeeChannelRepository,
            DoubleAngleRepository,
            HPileRepository,
            HollowStructuralSectionRepository,
            RoundHollowStructuralSectionRepository,
            MiscBeamRepository,
            MiscChannelRepository,
            MiscTeeRepository,
            PipeRepository,
            StructuralBeamRepository,
            StructuralTeeRepository,
            WideFlangeRepository,
            WideFlangeTeeRepository,
        >,
    >
{
    /// Creates a new instance of the service layer for the application
    /// using a Postgres implementation of AppState
    pub fn new(
        state: AppState<
            AngleRepository,
            CeeChannelRepository,
            DoubleAngleRepository,
            HPileRepository,
            HollowStructuralSectionRepository,
            RoundHollowStructuralSectionRepository,
            MiscBeamRepository,
            MiscChannelRepository,
            MiscTeeRepository,
            PipeRepository,
            StructuralBeamRepository,
            StructuralTeeRepository,
            WideFlangeRepository,
            WideFlangeTeeRepository,
        >,
    ) -> Self {
        PGShapeService { state }
    }
}

impl ShapeService
    for PGShapeService<
        AppState<
            AngleRepository,
            CeeChannelRepository,
            DoubleAngleRepository,
            HPileRepository,
            HollowStructuralSectionRepository,
            RoundHollowStructuralSectionRepository,
            MiscBeamRepository,
            MiscChannelRepository,
            MiscTeeRepository,
            PipeRepository,
            StructuralBeamRepository,
            StructuralTeeRepository,
            WideFlangeRepository,
            WideFlangeTeeRepository,
        >,
    >
{
    async fn angles(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::Angle>, Box<dyn std::error::Error>> {
        let angles: &[Angle] = &*self.state.angle_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::Angle> =
            angles.into_iter().map(|a| a.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn channels(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::CeeChannel>, Box<dyn std::error::Error>> {
        let channels: &[CeeChannel] = &*self.state.channel_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::CeeChannel> =
            channels.into_iter().map(|c| c.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn double_angles(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::DoubleAngle>, Box<dyn std::error::Error>> {
        let angles: &[DoubleAngle] = &*self.state.double_angle_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::DoubleAngle> =
            angles.into_iter().map(|d| d.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn h_piles(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::HPile>, Box<dyn std::error::Error>> {
        let h_piles: &[HPile] = &*self.state.h_pile_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::HPile> =
            h_piles.into_iter().map(|h| h.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn hss(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::HollowStructuralSection>, Box<dyn std::error::Error>>
    {
        let hss: &[HollowStructuralSection] = &*self.state.hss_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::HollowStructuralSection> =
            hss.into_iter().map(|h| h.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn misc_beams(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::MiscBeam>, Box<dyn std::error::Error>> {
        let misc_beams: &[MiscBeam] = &*self.state.misc_beam_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::MiscBeam> =
            misc_beams.into_iter().map(|m| m.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn misc_channels(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::MiscChannel>, Box<dyn std::error::Error>> {
        let misc_channels: &[MiscChannel] = &*self.state.misc_channel_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::MiscChannel> = misc_channels
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn misc_tees(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::MiscTee>, Box<dyn std::error::Error>> {
        let misc_tees: &[MiscTee] = &*self.state.misc_tee_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::MiscTee> =
            misc_tees.into_iter().map(|m| m.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn pipes(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::Pipe>, Box<dyn std::error::Error>> {
        let pipes: &[Pipe] = &*self.state.pipe_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::Pipe> =
            pipes.into_iter().map(|p| p.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn hss_round(
        &self,
    ) -> Result<
        Vec<crate::dto::aisc_shapes::RoundHollowStructuralSection>,
        Box<dyn std::error::Error>,
    > {
        let hss: &[RoundHollowStructuralSection] = &*self.state.hss_round_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::RoundHollowStructuralSection> =
            hss.into_iter().map(|h| h.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn structural_beams(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::StructuralBeam>, Box<dyn std::error::Error>> {
        let s_beams: &[StructuralBeam] = &*self.state.structural_beam_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::StructuralBeam> =
            s_beams.into_iter().map(|s| s.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn structural_tees(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::StructuralTee>, Box<dyn std::error::Error>> {
        let s_tees: &[StructuralTee] = &*self.state.structural_tee_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::StructuralTee> =
            s_tees.into_iter().map(|s| s.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn wide_flange_tees(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::WideFlangeTee>, Box<dyn std::error::Error>> {
        let wf_tees: &[WideFlangeTee] = &*self.state.wide_flange_tee_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::WideFlangeTee> =
            wf_tees.into_iter().map(|w| w.into()).collect::<Vec<_>>();
        Ok(dtos)
    }

    async fn wide_flange_beams(
        &self,
    ) -> Result<Vec<crate::dto::aisc_shapes::WideFlange>, Box<dyn std::error::Error>> {
        let beams: &[WideFlange] = &*self.state.wide_flange_repo.all().await?;
        let dtos: Vec<crate::dto::aisc_shapes::WideFlange> =
            beams.into_iter().map(|b| b.into()).collect::<Vec<_>>();
        Ok(dtos)
    }
}
